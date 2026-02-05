// ── Metrics page ────────────────────────────────────────────────
// Displays Prometheus metrics in a dashboard format with key metrics
// and links to external tools like Grafana.

import { signal } from "@preact/signals";
import { html } from "htm/preact";
import { render } from "preact";
import { useEffect, useState } from "preact/hooks";
import { registerPage } from "./router.js";

var metricsData = signal(null);
var loading = signal(true);
var error = signal(null);
var refreshInterval = null;

async function fetchMetrics() {
	try {
		var resp = await fetch("/api/metrics");
		if (!resp.ok) {
			if (resp.status === 503) {
				error.value = "Metrics are not enabled. Enable them in moltis.toml with [metrics] enabled = true";
			} else {
				error.value = "Failed to fetch metrics: " + resp.statusText;
			}
			return;
		}
		var data = await resp.json();
		metricsData.value = data;
		error.value = null;
	} catch (e) {
		error.value = "Failed to fetch metrics: " + e.message;
	} finally {
		loading.value = false;
	}
}

function formatNumber(n) {
	if (n === undefined || n === null) return "—";
	if (n >= 1000000) return (n / 1000000).toFixed(1) + "M";
	if (n >= 1000) return (n / 1000).toFixed(1) + "K";
	return n.toString();
}

function formatDuration(seconds) {
	if (seconds === undefined || seconds === null) return "—";
	if (seconds < 0.001) return "<1ms";
	if (seconds < 1) return Math.round(seconds * 1000) + "ms";
	if (seconds < 60) return seconds.toFixed(1) + "s";
	if (seconds < 3600) return Math.round(seconds / 60) + "m";
	return Math.round(seconds / 3600) + "h";
}

function formatUptime(seconds) {
	if (!seconds) return "—";
	var days = Math.floor(seconds / 86400);
	var hours = Math.floor((seconds % 86400) / 3600);
	var mins = Math.floor((seconds % 3600) / 60);
	if (days > 0) return days + "d " + hours + "h";
	if (hours > 0) return hours + "h " + mins + "m";
	return mins + "m";
}

function MetricCard({ title, value, subtitle, color }) {
	return html`
		<div class="bg-[var(--surface)] border border-[var(--border)] rounded-lg p-4">
			<div class="text-xs text-[var(--muted)] uppercase tracking-wide mb-1">${title}</div>
			<div class="text-2xl font-semibold" style=${color ? `color: ${color}` : ""}>${value}</div>
			${subtitle && html`<div class="text-xs text-[var(--muted)] mt-1">${subtitle}</div>`}
		</div>
	`;
}

function MetricsGrid({ categories }) {
	if (!categories) return null;

	var { llm, http, websocket, tools, mcp, session, system } = categories;

	return html`
		<div class="space-y-6">
			<!-- System Overview -->
			<section>
				<h3 class="text-sm font-medium text-[var(--muted)] uppercase tracking-wide mb-3">System</h3>
				<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
					<${MetricCard} title="Uptime" value=${formatUptime(system?.uptime_seconds)} />
					<${MetricCard} title="Connected Clients" value=${formatNumber(system?.connected_clients)} />
					<${MetricCard} title="Active Sessions" value=${formatNumber(system?.active_sessions)} />
					<${MetricCard} title="HTTP Requests" value=${formatNumber(http?.total)} />
				</div>
			</section>

			<!-- LLM Metrics -->
			<section>
				<h3 class="text-sm font-medium text-[var(--muted)] uppercase tracking-wide mb-3">LLM Usage</h3>
				<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
					<${MetricCard}
						title="Completions"
						value=${formatNumber(llm?.completions_total)}
						subtitle=${llm?.errors > 0 ? llm.errors + " errors" : undefined}
					/>
					<${MetricCard} title="Input Tokens" value=${formatNumber(llm?.input_tokens)} />
					<${MetricCard} title="Output Tokens" value=${formatNumber(llm?.output_tokens)} />
					<${MetricCard}
						title="Cache Tokens"
						value=${formatNumber((llm?.cache_read_tokens || 0) + (llm?.cache_write_tokens || 0))}
						subtitle=${llm?.cache_read_tokens ? "read: " + formatNumber(llm.cache_read_tokens) : undefined}
					/>
				</div>
			</section>

			<!-- Tools & MCP -->
			<section>
				<h3 class="text-sm font-medium text-[var(--muted)] uppercase tracking-wide mb-3">Tools & MCP</h3>
				<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
					<${MetricCard}
						title="Tool Executions"
						value=${formatNumber(tools?.total)}
						subtitle=${tools?.errors > 0 ? tools.errors + " errors" : undefined}
					/>
					<${MetricCard} title="Tools Active" value=${formatNumber(tools?.active)} />
					<${MetricCard}
						title="MCP Tool Calls"
						value=${formatNumber(mcp?.total)}
						subtitle=${mcp?.errors > 0 ? mcp.errors + " errors" : undefined}
					/>
					<${MetricCard} title="MCP Servers" value=${formatNumber(mcp?.active)} />
				</div>
			</section>

			<!-- Provider breakdown if available -->
			${llm?.by_provider && Object.keys(llm.by_provider).length > 0 && html`
				<section>
					<h3 class="text-sm font-medium text-[var(--muted)] uppercase tracking-wide mb-3">By Provider</h3>
					<div class="bg-[var(--surface)] border border-[var(--border)] rounded-lg overflow-hidden">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b border-[var(--border)] bg-[var(--surface2)]">
									<th class="text-left px-4 py-2 font-medium">Provider</th>
									<th class="text-right px-4 py-2 font-medium">Completions</th>
									<th class="text-right px-4 py-2 font-medium">Input Tokens</th>
									<th class="text-right px-4 py-2 font-medium">Output Tokens</th>
									<th class="text-right px-4 py-2 font-medium">Errors</th>
								</tr>
							</thead>
							<tbody>
								${Object.entries(llm.by_provider).map(([name, stats]) => html`
									<tr class="border-b border-[var(--border)] last:border-0">
										<td class="px-4 py-2">${name}</td>
										<td class="text-right px-4 py-2">${formatNumber(stats.completions)}</td>
										<td class="text-right px-4 py-2">${formatNumber(stats.input_tokens)}</td>
										<td class="text-right px-4 py-2">${formatNumber(stats.output_tokens)}</td>
										<td class="text-right px-4 py-2 ${stats.errors > 0 ? 'text-[var(--error)]' : ''}">${formatNumber(stats.errors)}</td>
									</tr>
								`)}
							</tbody>
						</table>
					</div>
				</section>
			`}
		</div>
	`;
}

function PrometheusEndpoint() {
	var [copied, setCopied] = useState(false);
	var endpoint = window.location.origin + "/metrics";

	function copyEndpoint() {
		navigator.clipboard.writeText(endpoint).then(() => {
			setCopied(true);
			setTimeout(() => setCopied(false), 2000);
		});
	}

	return html`
		<div class="mt-6 p-4 bg-[var(--surface)] border border-[var(--border)] rounded-lg">
			<h3 class="text-sm font-medium mb-2">Prometheus Endpoint</h3>
			<p class="text-xs text-[var(--muted)] mb-3">
				Scrape this endpoint with Prometheus or import into Grafana for advanced visualization.
			</p>
			<div class="flex items-center gap-2">
				<code class="flex-1 px-3 py-2 bg-[var(--surface2)] rounded text-sm font-mono">${endpoint}</code>
				<button
					class="provider-btn provider-btn-secondary text-sm"
					onClick=${copyEndpoint}
				>
					${copied ? "Copied!" : "Copy"}
				</button>
			</div>
		</div>
	`;
}

function MetricsPage() {
	useEffect(() => {
		fetchMetrics();
		// Refresh every 10 seconds
		refreshInterval = setInterval(fetchMetrics, 10000);
		return () => {
			if (refreshInterval) clearInterval(refreshInterval);
		};
	}, []);

	if (loading.value) {
		return html`
			<div class="p-8 text-center text-[var(--muted)]">
				Loading metrics...
			</div>
		`;
	}

	if (error.value) {
		return html`
			<div class="p-8">
				<div class="p-4 bg-[var(--error-bg)] border border-[var(--error)] rounded-lg text-[var(--error)]">
					${error.value}
				</div>
				<${PrometheusEndpoint} />
			</div>
		`;
	}

	return html`
		<div class="p-6">
			<div class="flex items-center justify-between mb-6">
				<h2 class="text-xl font-semibold">Metrics</h2>
				<button
					class="provider-btn provider-btn-secondary text-sm"
					onClick=${() => { loading.value = true; fetchMetrics(); }}
				>
					Refresh
				</button>
			</div>

			<${MetricsGrid} categories=${metricsData.value?.categories} />
			<${PrometheusEndpoint} />
		</div>
	`;
}

function init(container) {
	render(html`<${MetricsPage} />`, container);
}

function teardown() {
	if (refreshInterval) {
		clearInterval(refreshInterval);
		refreshInterval = null;
	}
	metricsData.value = null;
	loading.value = true;
	error.value = null;
}

registerPage("/metrics", init, teardown);
