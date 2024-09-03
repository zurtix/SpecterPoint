import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs"
import Terminal from '@/components/ui/terminal'
import { AgentItem } from "@/components/agent/item"
import { XIcon } from "lucide-react"
import { useEvents } from "../provider/events"

const agents = ["5ed36b39", "test"]

export function Agents() {
  const events = useEvents();

  function onMiddleClick(e: number, id: string, type: string) {
    if (e === 1) {
      events.removeInteraction(id, type)
    }
  }

  return (
    <ResizablePanelGroup
      direction="horizontal">
      <ResizablePanel defaultSize={10} minSize={17}>
        <div className="flex flex-col p-2 text-sm h-full overflow-y-scroll gap-2">
          {agents.map(id => <AgentItem id={id} onInteract={events.addInteraction} />)}
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={50}>
        <Tabs className="h-full w-full">
          <TabsList>
            {events.interactions?.map(inter =>
              <TabsTrigger value={`${inter.type}-${inter.id}`} className="w-full"
                onClick={(e) => onMiddleClick(e.button, inter.id, inter.type)}
              >
                <div className="flex w-full justify-between">
                  <p>{`${inter.type}: ${inter.id}`}</p>
                  <XIcon
                    className="mt-1"
                    height={12}
                    width={12}
                    onClick={() => events.removeInteraction(inter.id, inter.type)} />
                </div>
              </TabsTrigger>
            )}
          </TabsList>
          {events.interactions?.map(inter =>
            <TabsContent
              value={`${inter.type}-${inter.id}`}
              className="overflow-y-hidden h-full w-full">
              <Terminal
                id={inter.id}
                type={inter.type}
                history={inter.history}
                commands={inter.commands}
                onHistory={events.setHistory}
                onCommand={events.setCommand}
                onExit={events.removeInteraction}
              />
            </TabsContent>
          )}
        </Tabs>
      </ResizablePanel>
    </ResizablePanelGroup >
  )
}


