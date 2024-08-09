import { createFileRoute } from '@tanstack/react-router'
import { ServerPage } from "@/components/server/page"

export const Route = createFileRoute('/_auth/servers/view')({
  component: () => <ServerPage />
})

