import { Listeners } from '@/components/listener/page'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/listeners/view')({
  component: () => <Listeners />
})
