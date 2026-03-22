

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": true,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.4jmuQ8QQ.js","_app/immutable/chunks/LEypfyLm.js","_app/immutable/chunks/DQ32N8F_.js","_app/immutable/chunks/DCoxEa5e.js"];
export const stylesheets = ["_app/immutable/assets/0.BN16YHWY.css"];
export const fonts = [];
