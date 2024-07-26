import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/blacklist')({
  component: () => <div>Hello /blacklist!</div>
})
