import { useEffect, useState } from "react"
import { Separator } from "@/components/ui/separator"
import { ServerItem } from "@/components/server/item"
import { ServerView } from "@/components/server/view"
import { Input } from "@/components/ui/input"
import { ScrollArea } from "../ui/scroll-area"
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable"
import { Server } from "@/components/server/types"

const server_stub = [
  {
    id: 0,
    name: "server 0",
    running: 5,
    down: 2,
    type: "Manual"
  },
  {
    id: 1,
    name: "server 1",
    running: 5,
    down: 2,
    type: "Manual"
  },
  {
    id: 2,
    name: "server 2",
    running: 5,
    down: 2,
    type: "Manual"
  },
  {
    id: 3,
    name: "server 3",
    running: 5,
    down: 2,
    type: "Manual"
  }
]

export function ServerPage() {
  const [server, setServer] = useState<Server>();
  const [servers, setServers] = useState<Server[]>()

  function handleSearch(val: string) {
    setServers(server_stub.filter(s => s.name.includes(val)))
  }

  useEffect(() => {
    setServers(server_stub)
  }, [])

  return (
    <ResizablePanelGroup direction="horizontal">
      <ResizablePanel defaultSize={25}>
        <div className="flex flex-col p-2">
          <Input
            type="search"
            placeholder="Search"
            className=""
            onChange={(e) => handleSearch(e.currentTarget.value)} />
          <Separator className="mt-2" />
          <div className="supports-[backdrop-filter]:bg-background/60 overflow-y-scroll w-full">
            <ScrollArea className="w-full">          {
              servers?.map((s) => (
                <ServerItem
                  onClick={() => setServer(s)}
                  {...s}
                />
              ))
            }
            </ScrollArea>
          </div>
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={75}>
        <ServerView server={server} />
      </ResizablePanel>
    </ResizablePanelGroup>
  )
}

