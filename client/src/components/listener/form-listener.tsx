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
import { useEffect, useState } from "react"
import { ListenerTypes } from "@/types/listener"
import { Key, RefreshCw } from "lucide-react"
import { ResponsiveDialog } from "../ui/responsive-dialog"
import { Textarea } from "../ui/textarea"
import { Spinner } from "../ui/spinner"

export default function ({ setOpen }:
  { setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  const { toast } = useToast()
  const [endpoint, setEndpoint] = useState<string>("")
  const [metadata, setMetadata] = useState<string>("")
  const [viewKeys, setViewKeys] = useState(false)
  const [pending, setPending] = useState(true)
  const [keys, setKeys] = useState<[string, string]>(["", ""])
  const form = useForm({
    defaultValues: {
      type: "",
      name: "",
      host: "",
      port: 0,
      private_key: keys[0],
      public_key: keys[1],
      endpoints: [],
      metadata: []
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

  useEffect(() => {
    generateKeys()
  }, [])

  function generateKeys() {
    setPending(true)
    invoke<[string, string]>("generate_keys").then(keys => {
      setKeys(keys)
      setPending(false)
    }).catch((err) => (
      toast({
        variant: "destructive",
        title: "Failed to generate keys",
        description: err,
      })

    ))

  }

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

  function handleMetadata() {
    let name = metadata.substring(0, metadata.indexOf(':')).trim()
    let data = metadata.substring(metadata.indexOf(':') + 1).trim()
    let value = { "name": name, "data": data }
    let prev = form.getFieldValue("metadata")
    setMetadata("")

    // @ts-ignore
    if (prev.some(p => p.name == value.name && p.data == value.data)) return

    if (!prev) {
      // @ts-ignore
      form.setFieldValue("metadata", [value])
    }

    // @ts-ignore
    form.setFieldValue("metadata", [...prev, value])
  }

  return (
    <form onSubmit={(e) => {
      e.preventDefault()
      e.stopPropagation()
      form.handleSubmit()
    }}>
      <div>
        <ResponsiveDialog
          isOpen={viewKeys}
          setIsOpen={setViewKeys}
          title="RSA Key pairs"
          description="Below are default key pairs that can be overriden">
          <div className="flex flex-col">
            <form.Field
              name="private_key"
              children={(field) => (
                <FormItem field={field} label="Prviate Key">
                  <Textarea
                    rows={10}
                    id={field.name}
                    name={field.name}
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.currentTarget.value)} />
                </FormItem>
              )}
            />
            <form.Field
              name="public_key"
              children={(field) => (
                <FormItem field={field} label="Public Key">
                  <Textarea
                    rows={10}
                    id={field.name}
                    name={field.name}
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.currentTarget.value)} />
                </FormItem>
              )}
            />
            <div className="flex gap-2">
              <Button type="button" onClick={() => setViewKeys(false)}>Close</Button>
              <Button type="button" disabled={pending} variant="secondary" onClick={() => generateKeys()}>
                {pending ? <Spinner /> : <RefreshCw />}
              </Button>

            </div>
          </div>
        </ResponsiveDialog>
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
                onSubmit: z.array(z.string()).min(1)
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
                    <div className="h-[90px] overflow-scroll">
                      <div className="grid grid-cols-1 gap-2">
                        {field.state.value && field.state.value.map((value, idx) => (
                          value && <Badge variant="secondary" key={idx} className="cursor-pointer" onClick={() => field.removeValue(idx)}>{value}</Badge>
                        ))}
                      </div>
                    </div>
                  </div>
                </FormItem>
              )}
            />
            <form.Field
              name="metadata"
              children={(field) => (
                <FormItem field={field} label="Headers" description="Required headers for communication">
                  <div className="flex flex-col gap-2">
                    <div className="flex gap-2">
                      <Input
                        placeholder="Content-Type: application/json"
                        id={field.name}
                        name={field.name}
                        value={metadata}
                        onChange={(e) => setMetadata(e.currentTarget.value)}
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            e.preventDefault()
                            handleMetadata()
                          }
                        }}
                      />
                      <Button type="button" onClick={() => {
                        handleMetadata()
                      }}>+</Button>
                    </div>
                    <div className="h-[90px] overflow-scroll">
                      <div className="grid grid-cols-1 overflow-scroll">
                        {field.state.value && field.state.value.map((value: { name: string, data: string }, idx) => (
                          value &&
                          <div className="flex cursor-pointer w-full" onClick={() => field.removeValue(idx)}>
                            <Badge variant="secondary" className="break-keep" key={`name-${idx}`}>{value.name}</Badge>
                            <Badge key={`data-${idx}`}>{value.data}</Badge>
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                </FormItem>
              )}
            />
          </div>
        </div>
        <div className="flex gap-2 mt-4">
          <Button type="submit">Submit</Button>
          <Button type="button" disabled={pending} variant="secondary" onClick={() => setViewKeys(true)}>
            {pending ? <Spinner /> :
              <div className="flex gap-2">
                <Key width={20} height={20} />{"Keys"}
              </div>}
          </Button>
        </div>
      </div>
    </form>
  )
}
