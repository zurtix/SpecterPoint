import { createRootRoute, Outlet } from '@tanstack/react-router'

export const Route = createRootRoute({
  component: () => (
    <div>
      <Outlet />
      {/* <TanStackRouterDevtools position='bottom-right' /> */}
    </div>
  ),
})
