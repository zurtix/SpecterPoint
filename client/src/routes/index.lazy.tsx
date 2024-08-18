import { createLazyFileRoute } from '@tanstack/react-router'
import { Index } from "@/components/pages/index"

export const Route = createLazyFileRoute('/')({
  component: () => <Index />
})
