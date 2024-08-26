import { Server } from "@/components/server/types"
import { Button } from "@/components/ui/button"

export function ServerView({ server, remove }: { server?: Server, remove: (id: number) => void }) {

  return (
    (server &&
      <div className="w-full p-4">
        <Button type="button" onClick={() => remove(server.id)}>Delete</Button>
        <p>{server.id}</p>
      </div>)

  )
}
