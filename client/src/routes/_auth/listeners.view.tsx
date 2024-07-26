import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/listeners/view')({
  component: () => <div>Hello /listeners/view!</div>
})
