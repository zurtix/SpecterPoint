import { useForm } from "@tanstack/react-form"
import { FormItem } from "@/components/ui/tanstack-form"
import { zodValidator } from "@tanstack/zod-form-adapter"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { invoke } from "@tauri-apps/api/core"
import { useToast } from "@/components/ui/use-toast"
import { ServerScheme } from "@/types/server"

export function ManualServer({ setOpen }: { setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  const { toast } = useToast()
  const form = useForm({
    defaultValues: {
      name: "",
      scheme: "",
      host: "",
      port: 0,
      event_port: 0,
      username: "",
      password: "",
      type: "manual"
    },
    onSubmit: async ({ value }) => {
      invoke("add_server", { server: value }).then((_) => {
        setOpen(false)
        toast({
          variant: "default",
          title: "Successfully added server",
          description: `${value.name} was successfully added`,
        })
      }).catch((err) => (
        toast({
          variant: "destructive",
          title: "Failed to add server",
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
        <div className="col-span-2">
          <form.Field
            name="name"
            validators={{
              onChangeAsync: z.string().min(1, "A unique name is required"),
              onChangeAsyncDebounceMs: 500
            }}
            children={(field) => (
              <FormItem field={field}>
                <Input
                  id={field.name}
                  placeholder="Ghosty HTTP server"
                  name={field.name}
                  onChange={(e) => field.handleChange(e.target.value)}
                />
              </FormItem>
            )}
          />
        </div>
        <div className="flex gap-2 col-span-2">
          <form.Field
            name="scheme"
            validators={{
              onChangeAsync: z.string()
            }}
            children={(field) => (
              <FormItem field={field} className="w-[150px]">
                <Select defaultValue={field.state.value} onValueChange={(v) => field.handleChange(v)}>
                  <SelectTrigger>
                    <SelectValue placeholder="..." />
                  </SelectTrigger>
                  <SelectContent>
                    {(Object.values(ServerScheme) as Array<keyof typeof ServerScheme>).map((key) => (
                      <SelectItem value={key}>{key}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormItem>
            )}
          />

          <form.Field
            name="host"
            validators={{
              onChangeAsync: z.string().min(1, "Hostname or IP is required"),
              onChangeAsyncDebounceMs: 500
            }}
            children={(field) => (
              <FormItem
                field={field}
                className="w-[600px]">
                <Input
                  placeholder="127.0.0.1"
                  id={field.name}
                  name={field.name}
                  onChange={(e) => field.handleChange(e.target.value)}
                />
              </FormItem>
            )}
          />
          <form.Field
            name="port"
            validators={{
              onChangeAsync: z.number()
                .min(1, "Port range must be between 1-65535")
                .max(65535, "Port range must be between 1-65535"),
              onChangeAsyncDebounceMs: 200
            }}
            children={(field) => (
              <FormItem
                field={field}>
                <Input
                  type="number"
                  placeholder="80"
                  id={field.name}
                  name={field.name}
                  onChange={(e) => field.handleChange(e.target.valueAsNumber)}
                />
              </FormItem>
            )}
          />
          <form.Field
            name="event_port"
            validators={{
              onChangeAsync: z.number()
                .min(1, "Port range must be between 1-65535")
                .max(65535, "Port range must be between 1-65535"),
              onChangeAsyncDebounceMs: 200
            }}
            children={(field) => (
              <FormItem
                field={field}
                label="Event port">
                <Input
                  type="number"
                  placeholder="80"
                  id={field.name}
                  name={field.name}
                  onChange={(e) => field.handleChange(e.target.valueAsNumber)}
                />
              </FormItem>
            )}
          />

        </div>
        <form.Field
          name="username"
          validators={{
            onChangeAsync: z.string().min(1, "Username required to connect"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field}>
              <Input
                placeholder="ghosty"
                id={field.name}
                name={field.name}
                onChange={(e) => field.handleChange(e.target.value)}
              />
            </FormItem>
          )}
        />
        <form.Field
          name="password"
          validators={{
            onChangeAsync: z.string().min(1, "Password required to connect"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field}>
              <Input
                type="password"
                placeholder="******"
                id={field.name}
                name={field.name}
                onChange={(e) => field.handleChange(e.target.value)}
              />
            </FormItem>
          )}
        />
      </div>
      <div className="flex gap-2 mt-4 justify-between">
        <div className="flex gap-2">
          <Button type="submit">Connect</Button>
          <Popover>
            <PopoverTrigger asChild>
              <Button type="button" className="bg-secondary dark:text-white hover:bg-muted-foreground">Help?</Button>
            </PopoverTrigger>
            <PopoverContent>
              <p>
                To leverage the manual server registration, ensure you have a server deployed.
                To do so, run the server on a new host or your favorite cloud provider, then proceed with connecting.
                <br />
                <br />
                For additional help, please review the <a href="https://github.com/Surphix/SpecterPoint/wiki" className="underline">wiki</a>.
              </p>
            </PopoverContent>
          </Popover>
        </div>
        <Button type="button" variant="ghost" onClick={() => setOpen(false)}>Cancel</Button>
      </div>
    </form >
  )
}
