import { createRootRoute, Link, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import ghosty from '../assets/ghosty.gif'
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { ResponsiveDialog } from '@/components/ui/responsive-dialog'
import { useState } from 'react'
import ListenerForm from '@/components/listener-form'

export const Route = createRootRoute({
  component: () => (
    <div className='h-[calc(100vh-59px)]'>
      <Outlet />
      <Navigation />
      <TanStackRouterDevtools position='bottom-right' />
    </div>
  ),
})

function Navigation() {
  const [isCreateListenerOpen, setIsCreateListenerOpen] = useState(false);

  return (
    <>
      <ResponsiveDialog
        isOpen={isCreateListenerOpen}
        setIsOpen={setIsCreateListenerOpen}
        title="Create new Listener"
        description="HTTP (80) | HTTPS (443) | TCP (any)"
      >
        <ListenerForm />
      </ResponsiveDialog>

      <div className="z-0 p-2 fixed bottom-0 left-0 w-full">
        <hr className='mb-1' />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button className='bg-secondary'>
              <img src={ghosty} alt="ghost" width={40} height={40} />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-56" align='start' alignOffset={2}>
            <DropdownMenuLabel>Menu</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                <span>Listeners</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem asChild>
                    <span onClick={() => setIsCreateListenerOpen(true)}>Create</span>
                  </DropdownMenuItem>
                  <DropdownMenuItem asChild>
                    <Link to="/listeners/view" className='w-full'>View</Link>
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                <span>Servers</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem asChild>
                    <Link to="/servers/create" className='w-full'>Create</Link>
                  </DropdownMenuItem>
                  <DropdownMenuItem asChild>
                    <Link to="/servers/view" className='w-full'>View</Link>
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
            <DropdownMenuItem asChild>
              <Link to="/targets" className='w-full'>Targets</Link>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem asChild>
              <Link to='/blacklist'>Blacklist</Link>
            </DropdownMenuItem>
            <DropdownMenuItem asChild>
              <Link to='/configuration'>Configuration</Link>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem asChild>
              <a target="_blank" href="https://github.com/Surphix/SpecterPoint" className='w-full'>GitHub</a>
            </DropdownMenuItem>
            <DropdownMenuItem disabled>
              <span>Support</span>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
              <span>Quit</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </>
  )
}
