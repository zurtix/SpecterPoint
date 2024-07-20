import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/servers/create')({
  component: () => <div>Hello /servers/create!</div>
})