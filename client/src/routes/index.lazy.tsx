import { createLazyFileRoute } from '@tanstack/react-router'
import { Index } from "@/components/index"

export const Route = createLazyFileRoute('/')({
  component: () => <Index />
})
