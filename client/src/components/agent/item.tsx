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

export function AgentItem({ id, onInteract }: { id: string, onInteract: (id: string, type: string) => void }) {

  function handleDouble(e: React.MouseEvent<HTMLDivElement>) {
    if (e.detail === 2) {
      onInteract(id, "command")
    }
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger className="w-full hover:bg-secondary overflow-hidden">
        <Card className="cursor-pointer selecter-none" onClick={handleDouble}>
          <CardContent className="w-full p-2 text-center">
            <div>
              <p className="font-bold select-none">{id}</p>
              <div>
                <Badge className="bg-secondary text-muted-foreground select-none" variant="outline">2024-01-01 08:00</Badge>
              </div>
            </div>
          </CardContent>
        </Card>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem onClick={() => onInteract(id, "command")}>
          Interact
        </ContextMenuItem>
        <ContextMenuItem>
          Kill
        </ContextMenuItem>
        <ContextMenuItem>
          Update
        </ContextMenuItem>
        <ContextMenuItem onClick={() => onInteract(id, "shell")}>
          Remote Shell
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  )
}
