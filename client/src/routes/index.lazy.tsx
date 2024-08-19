import { createLazyFileRoute } from '@tanstack/react-router'
import { Index } from "@/components/index/page"

export const Route = createLazyFileRoute('/')({
  component: () => <Index />
})
