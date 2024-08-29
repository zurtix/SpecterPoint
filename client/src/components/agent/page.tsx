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

const agents = ["5ed36b39", "test"]

interface Interaction {
  history: string[],
  commands: string[],
  id: string,
  type: string,
}

export function Agents() {

  const [interactions, setInteractions] = useState<Interaction[]>([])

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
        commands: [],
        id: id,
        type: type
      }
      ]
    )
  }

  function setHistory(id: string, type: string, history: string[]) {
    let inters = [...interactions]
    let inter = interactions.find(it => it.id === id && it.type === type)

    if (inter) {
      inter.history = history
      setInteractions(inters)
    }
  }

  function setCommand(id: string, type: string, commands: string[]) {
    let inters = [...interactions]
    let inter = interactions.find(it => it.id === id && it.type === type)

    if (inter) {
      inter.commands = commands
      setInteractions(inters)
    }
  }

  function onMiddleClick(e: number, id: string, type: string) {
    if (e === 1) {
      remove(id, type)
    }
  }

  return (
    <ResizablePanelGroup
      direction="horizontal">
      <ResizablePanel defaultSize={10} minSize={17}>
        <div className="flex flex-col p-2 text-sm h-full overflow-y-scroll gap-2">
          {agents.map(id => <AgentItem id={id} onInteract={add} />)}
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={50}>
        <Tabs className="h-full w-full">
          <TabsList>
            {interactions?.map(inter =>
              <TabsTrigger value={`${inter.type}-${inter.id}`} className="w-full"
                onClick={(e) => onMiddleClick(e.button, inter.id, inter.type)}
              >
                <div className="flex w-full justify-between">
                  <p>{`${inter.type}: ${inter.id}`}</p>
                  <XIcon
                    className="mt-1"
                    height={12}
                    width={12}
                    onClick={() => remove(inter.id, inter.type)} />
                </div>
              </TabsTrigger>
            )}
          </TabsList>
          {interactions?.map(inter =>
            <TabsContent
              value={`${inter.type}-${inter.id}`}
              className="overflow-y-hidden h-full w-full">
              <Terminal
                id={inter.id}
                type={inter.type}
                history={inter.history}
                commands={inter.commands}
                onHistory={setHistory}
                onCommand={setCommand}
                onExit={remove}
              />
            </TabsContent>
          )}
        </Tabs>
      </ResizablePanel>
    </ResizablePanelGroup >
  )
}


