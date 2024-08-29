import { cn } from "@/lib/utils"
import { Listener } from "@/types/listener"

interface ListenerItemProps {
  listener: Listener
  onClick?: () => void
}

export function ListenerItem({ listener, onClick }: ListenerItemProps) {
  return (
    <button
      key={listener.id}
      className={cn(
        "flex flex-col w-full items-start gap-2 p-2 text-left text-sm transition-all hover:bg-accent",
      )}
      onClick={() => onClick && onClick()}
    >
      <div className="flex w-full flex-col gap-1">
        <div className="flex items-center w-full">
          <div className="flex items-center gap-2">
            <div className="font-semibold text-lg">{listener.name}</div>
          </div>
        </div>
      </div>
    </button>
  )
}
