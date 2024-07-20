import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/servers/view')({
  component: () => <div>Hello /servers/view!</div>
})