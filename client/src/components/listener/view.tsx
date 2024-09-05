import { Listener } from "@/types/listener"
import { Button } from "@/components/ui/button"
import { invoke } from "@tauri-apps/api/tauri"
import { useToast } from "../ui/use-toast"
import { Textarea } from "../ui/textarea"
import { Badge } from "../ui/badge"
import { Label } from "../ui/label"
import { Trash, Trash2 } from "lucide-react"

export function ListenerView({ listener, remove }: { listener?: Listener, remove: (id: number) => void }) {

  const { toast } = useToast()

  function start() {
    invoke("start_listener", { "id": listener?.id }).then(() => {
      toast({
        variant: "default",
        title: "Successfully deleted listener",
        description: `Successfully started listener [${listener?.id}]`
      })
    }).catch((err) => (
      toast({
        variant: "destructive",
        title: "Failed to start listener",
        description: err
      })
    ))
  }

  function stop() {
    invoke("stop_listener", { "id": listener?.id }).then(() => {
      toast({
        variant: "default",
        title: "Successfully deleted listener",
        description: `Successfully stopped listener [${listener?.id}]`
      })
    }).catch((err) => (
      toast({
        variant: "destructive",
        title: "Failed to stop listener",
        description: err
      })
    ))
  }

  return (
    (listener &&
      <div className="flex flex-col gap-4 w-full p-4">
        <div className="flex gap-2 justify-between">
          <div className="flex gap-2">
            <Button type="button" onClick={start} className="bg-secondary text-white">Start</Button>
            <Button type="button" onClick={stop} className="bg-red-950 text-white">Stop</Button>
          </div>
          <Trash2 className="text-red-500 opacity-75 hover:text-red-400 cursor-pointer" type="button" onClick={() => remove(listener.id)} />
        </div>
        <div className="flex flex-col gap-2">
          <Label className="font-bold">Endpoints</Label>
          <div className="flex">
            {listener.endpoints.map(endpoint => (<Badge>{endpoint.endpoint}</Badge>))}
          </div>
        </div>
        <div className="flex flex-col gap-2">
          <Label className="font-bold">Keys</Label>
          <Textarea rows={10}>{listener.public_key}</Textarea>
          <Textarea rows={10}>{listener.private_key}</Textarea>
        </div>
      </div>)

  )
}
