import { createFileRoute } from '@tanstack/react-router'
import { Servers } from "@/components/server/page"

export const Route = createFileRoute('/_auth/servers/view')({
  component: () => <Servers />
})

