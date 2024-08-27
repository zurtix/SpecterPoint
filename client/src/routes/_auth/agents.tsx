import { Agents } from '@/components/agent/page'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/agents')({
  component: () => <Agents />
})

