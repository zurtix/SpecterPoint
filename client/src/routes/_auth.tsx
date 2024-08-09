import { Outlet, createFileRoute } from '@tanstack/react-router'
import { Navigation } from "@/components/navigation"

export const Route = createFileRoute('/_auth')({
  component: () => (
    <div className="h-[calc(100vh-59px)] w-full flex flex-row">
      <Outlet />
      <Navigation />
    </div>
  )
})


