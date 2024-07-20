import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/blacklist')({
  component: () => <div>Hello /blacklist!</div>
})
