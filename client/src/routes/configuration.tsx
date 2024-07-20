import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/configuration')({
  component: () => <div>Hello /configuration!</div>
})
