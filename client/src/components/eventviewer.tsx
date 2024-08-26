import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { cn } from '@/lib/utils'
import { DropdownMenuCheckboxItemProps } from "@radix-ui/react-dropdown-menu"
import { Label } from '@/components/ui/label'
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"

type Checked = DropdownMenuCheckboxItemProps["checked"]

interface LogMessage {
  timestamp: string,
  level: string,
  message: string
}

export function EventViewer() {

  const [lines, setLines] = useState<LogMessage[]>([]);
  const [trace, setTrace] = useState<Checked>(false);
  const [debug, setDebug] = useState<Checked>(true);
  const [info, setInfo] = useState<Checked>(true);
  const [error, setError] = useState<Checked>(true);


  useEffect(() => {
    const unlisten = listen<LogMessage>('log-event', (event) => {
      if (event.payload.level === "TRACE" && trace
        || event.payload.level === "DEBUG" && debug
        || event.payload.level === "INFO" && info
        || event.payload.level === "ERROR" && error) {
        setLines((prevLines) => {
          const newLines = [...prevLines, event.payload];
          if (newLines.length > 1000) {
            newLines.splice(0, newLines.length - 1000);
          }
          return newLines;
        });
      }
    });

    return () => {
      unlisten.then((off) => off());
    };
  }, [trace, debug, info, error]);

  return (
    <div className="flex flex-col h-full w-full overflow-hidden">
      <div className="text-center">
        <Label>Event messages</Label>
      </div>
      <div className="overflow-y-scroll h-full w-full">
        {lines.map((line) => (
          <p className="text-xs">
            {line.timestamp} : <span className={cn(line.level === "DEBUG" ? "text-green-500" : "", line.level === "ERROR" ? "text-red-500" : "")}>{line.level}</span> - {line.message}
          </p>
        ))}
      </div>
      <div className="text-right">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="secondary" className="h-6 w-12">Filter</Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-56">
            <DropdownMenuCheckboxItem
              checked={trace}
              onCheckedChange={setTrace}
            >
              TRACE
            </DropdownMenuCheckboxItem>
            <DropdownMenuCheckboxItem
              checked={debug}
              onCheckedChange={setDebug}
            >
              DEBUG
            </DropdownMenuCheckboxItem>
            <DropdownMenuCheckboxItem
              checked={info}
              onCheckedChange={setInfo}
            >
              INFO
            </DropdownMenuCheckboxItem>
            <DropdownMenuCheckboxItem
              checked={error}
              onCheckedChange={setError}
            >
              ERROR
            </DropdownMenuCheckboxItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  )
}
