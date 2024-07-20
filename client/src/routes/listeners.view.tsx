import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/listeners/view')({
  component: () => <div>Hello /listeners/view!</div>
})
