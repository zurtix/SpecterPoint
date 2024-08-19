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
import { invoke } from "@tauri-apps/api/tauri"

export function Servers() {
  const [server, setServer] = useState<Server>();
  const [servers, setServers] = useState<Server[]>()

  useEffect(() => {
    invoke<Server[]>("all_servers").then((srvs) => (
      setServers(srvs)
    ))
  }, [])


  function handleSearch(val: string) {
    setServers(servers!.filter(s => s.name.includes(val)))
  }

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
                  key={s.id}
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

