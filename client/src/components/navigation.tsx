import { Link } from "@tanstack/react-router"
import ghosty from '@/assets/ghosty.gif'
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useEffect, useState } from 'react'
import { CreateServer } from "@/components/server/create"
import { invoke } from '@tauri-apps/api/core'
import { CreateListener } from "./listener/create"
import { Badge } from "./ui/badge"

export function Navigation() {
  const [isCreateListenerOpen, setIsCreateListenerOpen] = useState(false);
  const [isCreateServerOpen, setIsCreateServerOpen] = useState(false);
  const [count, setCount] = useState<number>(0)

  useEffect(() => {
    invoke<number>("agent_count")
      .then((c) => setCount(c))
      .catch(() => setCount(0))
  }, [count])

  return (
    <div>
      <CreateListener open={isCreateListenerOpen} setOpen={setIsCreateListenerOpen} />
      <CreateServer open={isCreateServerOpen} setOpen={setIsCreateServerOpen} />
      <div className="z-70 p-2 fixed bottom-0 left-0 w-full">
        <hr className='mb-1' />
        <div className="flex">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button className='bg-secondary'>
                <img src={ghosty} alt="ghost" width={40} height={40} />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56" align='start' alignOffset={2}>
              <DropdownMenuItem>
                <Link to="/agents" className='w-full'>
                  <div className="flex justify-between">
                    <p>Agents</p>
                    <Badge className="bg-muted-foreground rounded">{count}</Badge>
                  </div>
                </Link>
              </DropdownMenuItem>
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
                      <span onClick={() => setIsCreateServerOpen(true)}>Create</span>
                    </DropdownMenuItem>
                    <DropdownMenuItem asChild>
                      <Link to="/servers/view" className='w-full'>View</Link>
                    </DropdownMenuItem>
                  </DropdownMenuSubContent>
                </DropdownMenuPortal>
              </DropdownMenuSub>
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
                <span className="w-full" onClick={() => invoke("quit")}>Quit</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </div>
  )
}
