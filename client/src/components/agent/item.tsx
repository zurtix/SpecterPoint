import {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from "@/components/ui/context-menu"

export function AgentItem({ id, onInteract }: { id: string, onInteract: (id: string, type: string) => void }) {
  return (
    <ContextMenu>
      <ContextMenuTrigger className="w-full hover:bg-secondary">
        {id}
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
    </ContextMenu >
  )
}
