<script lang="ts">
	import './layout.css';

	const { children } = $props();
</script>

<div class="background-grid"></div>

{@render children()}

<style>
    .background-grid {
        position: fixed;
        inset: 0;
        z-index: -1;
        background-image: linear-gradient(oklch(from var(--foreground) l c h / 0.2) 1px, transparent 1px),
        linear-gradient(90deg, oklch(from var(--foreground) l c h / 0.2) 1px, transparent 1px);
        background-size: 33px 33px;
        opacity: 0.3;
    }

</style>