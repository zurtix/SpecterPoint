import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu"
import {
  Card, CardContent
} from "@/components/ui/card"
import { Badge } from "../ui/badge"
import { Agent } from "@/types/agent"
import { format } from "date-fns"

export function AgentItem({ agent, onInteract }: { agent: Agent, onInteract: (id: string, type: string) => void }) {

  function handleDouble(e: React.MouseEvent<HTMLDivElement>) {
    if (e.detail === 2) {
      onInteract(agent.id, "command")
    }
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger className="w-full hover:bg-secondary overflow-hidden">
        <Card className="cursor-pointer selecter-none" onClick={handleDouble}>
          <CardContent className="w-full p-2 text-center">
            <div>
              <p className="font-bold select-none">{agent.id}</p>
              <div>
                <Badge className="bg-secondary text-muted-foreground select-none" variant="outline">{format(agent.last_seen, "yyy-MM-dd HH:mm:ss")}</Badge>
              </div>
            </div>
          </CardContent>
        </Card>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem onClick={() => onInteract(agent.id, "command")}>
          Interact
        </ContextMenuItem>
        <ContextMenuItem>
          Kill
        </ContextMenuItem>
        <ContextMenuItem>
          Update
        </ContextMenuItem>
        <ContextMenuItem onClick={() => onInteract(agent.id, "shell")}>
          Remote Shell
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  )
}
