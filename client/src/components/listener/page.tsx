import { useEffect, useState } from "react"
import { Separator } from "@/components/ui/separator"
import { ListenerItem } from "@/components/listener/item"
import { ListenerView } from "@/components/listener/view"
import { Input } from "@/components/ui/input"
import { ScrollArea } from "../ui/scroll-area"
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable"
import { Server } from "@/types/server"
import { invoke } from "@tauri-apps/api/tauri"
import { useToast } from "../ui/use-toast"
import { Listener } from "@/types/listener"

export function Listeners() {
  const [listener, setListener] = useState<Listener>()
  const [listeners, setListeners] = useState<Listener[]>()
  const { toast } = useToast();

  useEffect(() => {
    invoke<Server[]>("all_listeners").then((lstnrs) => (
      setListeners(lstnrs)
    ))
  }, [listeners])


  function remove(id: number) {
    invoke("remove_listener", { "id": id }).then(() => {
      setListeners(prev => prev?.filter(s => s.id === id))
      setListener(undefined)
      toast({
        variant: "default",
        title: "Successfully deleted listener",
        description: `Successfully removed listener [${id}]`
      })
    }).catch((err) => (
      toast({
        variant: "destructive",
        title: "Failed to deleted listener",
        description: err
      })
    ))
  }

  function handleSearch(val: string) {
    setListeners(listeners!.filter(s => s.name.includes(val)))
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
              listeners?.map((l) => (
                <ListenerItem
                  key={l.id}
                  onClick={() => setListener(l)}
                  listener={l}
                />
              ))
            }
            </ScrollArea>
          </div>
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={75}>
        <ListenerView listener={listener} remove={remove} />
      </ResizablePanel>
    </ResizablePanelGroup>
  )
}

