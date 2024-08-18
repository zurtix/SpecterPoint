import { cn } from "@/lib/utils"
import { Badge } from "@/components/ui/badge"

interface ServerItemProps {
  id: number,
  name: String,
  running?: number,
  down?: number,
  type?: String,
  onClick?: () => void
}

export function ServerItem({ id, name, running, down, type, onClick }: ServerItemProps) {
  return (
    <button
      key={id}
      className={cn(
        "flex flex-col w-full items-start gap-2 p-2 text-left text-sm transition-all hover:bg-accent",
      )}
      onClick={() => onClick && onClick()}
    >
      <div className="flex w-full flex-col gap-1">
        <div className="flex items-center w-full">
          <div className="flex items-center gap-2">
            <div className="font-semibold text-lg">{name}</div>
          </div>
        </div>
      </div>
      <div className="flex items-center gap-2">
        <Badge className="bg-green-500">
          {running ? (running) : ("?")}
        </Badge>
        <Badge className="bg-red-500">
          {down ? (down) : ("?")}
        </Badge>
        <Badge className="bg-muted text-gray-500">
          {type}
        </Badge>
      </div>
    </button>

  )
}
