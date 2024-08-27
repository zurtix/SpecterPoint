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
import { MultiBadge } from "@/components/ui/multi-badge"
import { useToast } from "@/components/ui/use-toast"
import { invoke } from "@tauri-apps/api/tauri"

enum ListenerTypes {
  Http = "Http",
  Https = "Https",
  Tcp = "Tcp",
}

export default function ({ setOpen }:
  { setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  const { toast } = useToast()
  const form = useForm({
    defaultValues: {
      type: "",
      name: "",
      host: "",
      port: 0,
      endpoints: [""]
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
              <FormItem field={field} description="Remote port number">
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
            children={(field) => (
              <FormItem field={field} description="Endpoints for agent communications">
                <MultiBadge
                  onValueChange={(v) => field.handleChange(v)}
                  placeholder="Enter endpoints"
                />
              </FormItem>
            )}
          />
        </div>
      </div>
      <Button type="submit" className="mt-4">Submit</Button>
    </form>
  )
}
