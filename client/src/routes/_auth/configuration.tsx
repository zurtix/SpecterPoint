import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/configuration')({
  component: () => <div>Hello /configuration!</div>
})
