import { useForm } from "@tanstack/react-form"
import { zodValidator } from "@tanstack/zod-form-adapter"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import { FormItem } from "@/components/ui/tanstack-form"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { useToast } from "@/components/ui/use-toast"
import { invoke } from "@tauri-apps/api/tauri"
import { Badge } from "@/components/ui/badge"
import { useState } from "react"
import { ListenerTypes } from "@/types/listener"

export default function ({ setOpen }:
  { setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  const { toast } = useToast()
  const [endpoint, setEndpoint] = useState<string>("");
  const form = useForm({
    defaultValues: {
      type: "",
      name: "",
      host: "",
      port: 0,
      endpoints: []
    },
    onSubmit: async ({ value }) => {
      invoke("add_listener", { create: value }).then((_) => {
        setOpen(false);
        toast({
          variant: "default",
          title: "Successfully added listener",
          description: `${value.name} was successfully added`,
        })
      }).catch((err) => (
        toast({
          variant: "destructive",
          title: "Failed to add listener",
          description: err,
        })
      ))
    },
    validatorAdapter: zodValidator()
  })

  function handleEndpoint() {
    let value = endpoint.startsWith('/') ? endpoint : '/' + endpoint
    let prev = form.getFieldValue("endpoints")
    setEndpoint("")

    // @ts-ignore
    if (prev.includes(value)) return

    if (!prev) {
      // @ts-ignore
      form.setFieldValue("endpoints", [value])
    }

    // @ts-ignore
    form.setFieldValue("endpoints", [...prev, value])
  }

  return (
    <form onSubmit={(e) => {
      e.preventDefault()
      e.stopPropagation()
      form.handleSubmit()
    }}>
      <div className="grid grid-cols-2 justify-between gap-4">
        <div>
          <form.Field
            name="type"
            validators={{
              onChangeAsync: z.string()
            }}
            children={(field) => (
              <FormItem field={field} description="HTTP | HTTPS | TCP">
                <Select defaultValue={field.state.value} onValueChange={(v) => field.handleChange(v)}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select a listener type" />
                  </SelectTrigger>
                  <SelectContent>
                    {(Object.values(ListenerTypes) as Array<keyof typeof ListenerTypes>).map((key) => (
                      <SelectItem value={key}>{key}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormItem>
            )}
          />
          <form.Field
            name="name"
            validators={{
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.string().min(1, "A name must be provided"),
            }}
            children={(field) => (
              <FormItem field={field} description="Unique listener name">
                <Input
                  placeholder="Ghosty Listener"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)} />
              </FormItem>
            )}
          />
          <form.Field
            name="host"
            validators={{
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.string().min(1, "A host must be provided"),
            }}
            children={(field) => (
              <FormItem field={field} description="Remote hostname or IP">
                <Input
                  placeholder="127.0.0.1"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)} />
              </FormItem>
            )}
          />
          <form.Field
            name="port"
            validators={{
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.number().min(1, "A valid port must be provided")
                .max(65535, "A valid port must be provided"),
            }}
            children={(field) => (
              <FormItem field={field} description="Port for the listener to listen on">
                <Input
                  type="number"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.valueAsNumber)} />
              </FormItem>
            )}
          />
        </div>
        <div className="w-full">
          <form.Field
            name="endpoints"
            validators={{
            }}
            children={(field) => (
              <FormItem field={field} description="Endpoints for agent communications">
                <div className="flex flex-col gap-2">
                  <div className="flex gap-2">
                    <Input
                      placeholder="/specterpoint/index.html"
                      id={field.name}
                      name={field.name}
                      value={endpoint}
                      onChange={(e) => setEndpoint(e.currentTarget.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") {
                          e.preventDefault()
                          handleEndpoint()
                        }
                      }}
                    />
                    <Button type="button" onClick={() => {
                      handleEndpoint()
                    }}>+</Button>
                  </div>
                  <div className="flex gap-2">
                    {field.state.value && field.state.value.map((value, idx) => (
                      value && <Badge key={idx} className="cursor-pointer" onClick={() => field.removeValue(idx)}>{value}</Badge>
                    ))}
                  </div>
                </div>
              </FormItem>
            )}
          />
        </div>
      </div>
      <Button type="submit" className="mt-4">Submit</Button>
    </form >
  )
}
