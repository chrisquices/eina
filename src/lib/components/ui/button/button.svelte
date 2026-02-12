<script lang="ts" module>
    import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements";
    import { tv,type VariantProps } from "tailwind-variants";

    import { cn, type WithElementRef } from "$lib/utils.js";

    export const buttonVariants = tv({
        base: "cursor-pointer focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 rounded-xl  font-medium whitespace-nowrap transition-all outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        variants: {
            variant: {
                default: "bg-primary text-primary-foreground hover:bg-primary/80 shadow-xs [&_svg]:!text-primary-foreground border border-primary hover:border-primary/0",
                success: "bg-success text-success-foreground hover:bg-success/60 shadow-xs [&_svg]:!text-success-foreground border border-success",
                destructive:
                    "bg-destructive hover:bg-destructive/60 focus-visible:ring-destructive/20 text-destructive-foreground shadow-xs border border-destructive",
                outline:
                    "bg-transparent hover:!bg-primary hover:text-primary-foreground border border-border shadow-xs",
                secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-xs border border-secondary",
                ghost: "hover:bg-primary hover:text-primary-foreground",
                link: "text-primary underline-offset-4 hover:underline",
                accent: "bg-accent text-accent-foreground hover:bg-primary/60 shadow-xs border border-accent",
                muted: "bg-muted text-muted-foreground hover:bg-muted/60 shadow-xs border border-muted",
            },
            size: {
                default: "h-10 px-4 has-[>svg]:px-4",
                xs: "h-6 gap-2 text-xs rounded-xl px-3 has-[>svg]:px-3",
                sm: "h-8 gap-2 text-xs rounded-xl px-3 has-[>svg]:px-3",
                lg: "h-12 rounded-xl px-6 has-[>svg]:px-4",
                icon: "size-10",
                "icon-sm": "size-8",
                "icon-md": "size-9",
                "icon-lg": "size-10",
                "icon-xl": "size-12",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    });

    export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
    export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

    export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
        WithElementRef<HTMLAnchorAttributes> & {
        variant?: ButtonVariant;
        size?: ButtonSize;
        rounded?: boolean;
    };
</script>

<script lang="ts">
    let {
        class: className,
        variant = "default",
        size = "default",
        rounded = true,
        ref = $bindable(null),
        href = undefined,
        type = "button",
        disabled,
        children,
        ...restProps
    }: ButtonProps = $props();
</script>

{#if href}
    <a
            bind:this={ref}
            data-slot="button"
            class={cn(buttonVariants({ variant, size }), rounded ? '' : '!rounded-none', className)}
            href={disabled ? undefined : href}
            aria-disabled={disabled}
            role={disabled ? "link" : undefined}
            tabindex={disabled ? -1 : undefined}
            {...restProps}
    >
        {@render children?.()}
    </a>
{:else}
    <button
            bind:this={ref}
            data-slot="button"
            class={cn(buttonVariants({ variant, size }), rounded ? '' : '!rounded-none', className)}
            {type}
            {disabled}
            {...restProps}
    >
        {@render children?.()}
    </button>
{/if}
