import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/servers/view')({
  component: () => <div>Hello /servers/view!</div>
})