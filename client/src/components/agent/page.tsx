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
import { useState } from "react"
import { XIcon } from "lucide-react"

const agents = ["5ed36b39eb1da5a6bcfdaa2a45df84ac"]

interface Interaction {
  history: string[],
  id: string,
  type: string,
}

export function Agents() {

  const [interactions, setInteractions] = useState<Interaction[]>([]);

  function remove(id: string, type: string) {
    setInteractions(prev => prev.filter(p => p.id !== id || p.type != type))
  }

  function add(id: string, type: string) {
    if (interactions.find(inter => inter.id == id && inter.type == type)) {
      return
    }

    setInteractions(prev =>
      [...prev,
      {
        history: [`${type} is now open for agent ${id}`],
        id: id,
        type: type
      }
      ])
  }

  function setHistory(id: string, type: string, history: string[]) {
    let inters = [...interactions]
    let inter = interactions.find(it => it.id === id && it.type === type)

    if (inter) {
      inter.history = history
      setInteractions(inters)
    }
  }

  return (
    <ResizablePanelGroup
      direction="horizontal">
      <ResizablePanel defaultSize={10}>
        <div className="flex flex-col p-2 text-sm h-full overflow-y-scroll">
          {agents.map(id => <AgentItem id={id} onInteract={add} />)}
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={50}>
        <Tabs className="h-full w-full">
          <TabsList>
            {interactions?.map(inter =>
              <TabsTrigger value={`${inter.type}-${inter.id}`} className="flex gap-2">
                {`${inter.type}: ${inter.id}`}
                <XIcon
                  height={12}
                  width={12}
                  onClick={() => remove(inter.id, inter.type)} />
              </TabsTrigger>
            )}
          </TabsList>
          {interactions?.map(inter =>
            <TabsContent value={`${inter.type}-${inter.id}`} className="overflow-y-hidden h-full w-full" >
              <Terminal
                id={inter.id}
                type={inter.type}
                history={inter.history}
                onHistory={setHistory}
                onExit={remove}
              />
            </TabsContent>
          )}
        </Tabs>
      </ResizablePanel>
    </ResizablePanelGroup >
  )

}


