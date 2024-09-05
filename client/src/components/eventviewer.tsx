import { useState } from 'react'
import { cn } from '@/lib/utils'
import { Label } from '@/components/ui/label'
import { Button } from "@/components/ui/button"
import { format } from 'date-fns'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Separator } from '@/components/ui/separator'
import { useEvents } from './provider/events'


export function EventViewer() {

  const [levels, setLevels] = useState<string[]>(["DEBUG", "INFO", "ERROR"])
  const events = useEvents()

  function levelChanged(e: boolean, level: string) {
    if (e && !levels.includes(level)) {
      setLevels(prev => [...prev, level])
    }

    if (!e && levels.includes(level)) {
      setLevels(prev => prev.filter(p => p != level))
    }
  }

  return (
    <div className="flex flex-col h-full w-full overflow-hidden">
      <div className="text-center">
        <div className="p-2">
          <div className="flex justify-between">
            <Label>Event Viewer</Label>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" className="h-4 w-12 p-2">Filter</Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent className="w-56">
                <DropdownMenuCheckboxItem
                  checked={levels.includes("TRACE")}
                  onCheckedChange={(e) => levelChanged(e, "TRACE")}
                >
                  TRACE
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  checked={levels.includes("DEBUG")}
                  onCheckedChange={(e) => levelChanged(e, "DEBUG")}
                >
                  DEBUG
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  checked={levels.includes("INFO")}
                  onCheckedChange={(e) => levelChanged(e, "INFO")}
                >
                  INFO
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  checked={levels.includes("ERROR")}
                  onCheckedChange={(e) => levelChanged(e, "ERROR")}
                >
                  ERROR
                </DropdownMenuCheckboxItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
        <Separator className="text-muted" />
      </div>
      <div className="overflow-y-scroll h-full w-full p-2">
        {events.logs.filter(log => levels.includes(log.level)).map((line, idx) => (
          <p key={`${idx}`} className="text-xs" >
            {format(line.timestamp, "yyyy/MM/dd HH:mm:ss")} : <span className={cn(line.level === "DEBUG" ? "text-green-500" : "", line.level === "ERROR" ? "text-red-500" : "")}>[{line.level}]</span> - {line.message}
          </p>
        ))}
      </div>
    </div >
  )
}
