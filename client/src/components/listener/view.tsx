import { Listener } from "@/components/listener/types"
import { Button } from "@/components/ui/button"
import { invoke } from "@tauri-apps/api/tauri"
import { useToast } from "../ui/use-toast"

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
      <div className="w-full p-4">
        <Button type="button" onClick={() => remove(listener.id)}>Delete</Button>
        <Button type="button" onClick={start} className="bg-green-500">Start</Button>
        <Button type="button" onClick={stop} className="bg-red-500">Stop</Button>
        <p>{listener.id}</p>
      </div>)

  )
}
