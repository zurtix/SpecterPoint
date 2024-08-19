import { useForm } from "@tanstack/react-form"
import { zodValidator } from "@tanstack/zod-form-adapter"
import { z } from "zod"
import { cn } from "@/lib/utils"
import { FormItem } from "@/components/ui/tanstack-form"
import { Check, ChevronsUpDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import { Separator } from "@/components/ui/separator"
import { Input } from "@/components/ui/input"

const aws_regions = [
  "us-east-2",
  "us-east-1",
  "us-west-1",
  "us-west-2",
  "af-south-1",
  "ap-east-1",
  "ap-south-2",
  "ap-southeast-3",
  "ap-southeast-4",
  "ap-south-1",
  "ap-northeast-3",
  "ap-northeast-2",
  "ap-southeast-1",
  "ap-southeast-2",
  "ap-northeast-1",
  "ca-central-1",
  "ca-west-1",
  "eu-central-1",
  "eu-west-1",
  "eu-west-2",
  "eu-south-1",
  "eu-west-3",
  "eu-south-2",
  "eu-north-1",
  "eu-central-2",
  "il-central-1",
  "me-south-1",
  "me-central-1",
  "sa-east-1",
]

export function AwsServer({ setOpen }:
  { setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  const form = useForm({
    defaultValues: {
      endpoint: "",
      region: "",
      accessKey: "",
      accessSecret: "",
      username: "",
      password: ""
    },
    onSubmit: async ({ value }) => {
      console.log(value)
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
        <form.Field
          name="endpoint"
          validators={{
            onChangeAsync: z.string().min(1, "Endpoint required"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field} description="AWS Endpoint">
              <Input
                placeholder="amazonaws.com"
                id={field.name}
                name={field.name}
                onChange={(e) => field.handleChange(e.target.value)}
              />
            </FormItem>
          )}
        />
        <form.Field
          name="region"
          children={(field) => (
            <FormItem field={field} description="AWS Geo region">
              <Popover>
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    role="combobox"
                    className={cn(
                      "w-[200px] justify-between",
                      !field.state.value && "text-muted-foreground"
                    )}
                  >
                    {field.state.value
                      ? aws_regions.find(
                        (region) => region === field.state.value
                      )
                      : "Select region"}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-[200px] p-0">
                  <Command>
                    <CommandInput placeholder="Search region..." />
                    <CommandList>
                      <CommandEmpty>No region found</CommandEmpty>
                      <CommandGroup>
                        {aws_regions.map((region) => (
                          <CommandItem
                            style={{ pointerEvents: "auto" }}
                            value={region}
                            key={region}
                            onSelect={() => field.handleChange(region)}
                          >
                            <Check
                              className={cn(
                                "mr-2 h-4 w-4",
                                region === field.state.value
                                  ? "opacity-100"
                                  : "opacity-0"
                              )}
                            />
                            {region}
                          </CommandItem>
                        ))}
                      </CommandGroup>
                    </CommandList>
                  </Command>
                </PopoverContent>
              </Popover>
            </FormItem>
          )}
        />
        <form.Field
          name="accessKey"
          validators={{
            onChangeAsync: z.string().min(1, "Access key is required"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field} label="Access Key" description="Key ID for AWS">
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
        <form.Field
          name="accessSecret"
          validators={{
            onChangeAsync: z.string().min(1, "Access secret is required"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field} label="Access Secret" description="Secret access key for AWS">
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
        <Separator className="col-span-2" />
        <form.Field
          name="username"
          validators={{
            onChangeAsync: z.string().min(1, "Username is required"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field} label="New server username" description="New username to configure server for auth">
              <Input
                type="text"
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
            onChangeAsync: z.string().min(1, "Password is reqiured"),
            onChangeAsyncDebounceMs: 500
          }}
          children={(field) => (
            <FormItem field={field} label="New server password" description="New password to configure server for auth">
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
              <p>To be implemented!</p>
            </PopoverContent>
          </Popover>
        </div>
        <Button type="button" variant="ghost" onClick={() => setOpen(false)}>Cancel</Button>
      </div>
    </form>
  )
}
