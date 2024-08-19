import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { cn } from "@/lib/utils"

import { Check, ChevronsUpDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
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

const awsServerSchema = z.object({
  endpoint: z.string(),
  region: z.string(),
  accessKey: z.string(),
  accessSecret: z.string(),
  username: z.string(),
  password: z.string()
})

export function AwsServer() {
  const form = useForm<z.infer<typeof awsServerSchema>>({
    resolver: zodResolver(awsServerSchema),
  })

  function onSubmit(values: z.infer<typeof awsServerSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    console.log(values)
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div className="grid grid-cols-2 justify-between gap-4">
          <FormField
            control={form.control}
            name="endpoint"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Endpoint</FormLabel>
                <FormDescription>
                  AWS Endpoint
                </FormDescription>
                <FormControl>
                  <Input placeholder="amazonaws.com" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="region"
            render={({ field }) => (
              <FormItem className="flex flex-col mt-2">
                <FormLabel>Region</FormLabel>
                <FormDescription>
                  Geographical region of your subscription
                </FormDescription>
                <Popover>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant="outline"
                        role="combobox"
                        className={cn(
                          "w-[200px] justify-between",
                          !field.value && "text-muted-foreground"
                        )}
                      >
                        {field.value
                          ? aws_regions.find(
                            (region) => region === field.value
                          )
                          : "Select region"}
                        <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                      </Button>
                    </FormControl>
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
                              onSelect={() => {
                                form.setValue("region", region)
                              }}
                            >
                              <Check
                                className={cn(
                                  "mr-2 h-4 w-4",
                                  region === field.value
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
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="accessKey"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Access key ID</FormLabel>
                <FormDescription>
                  Key ID for remote auth
                </FormDescription>
                <FormControl>
                  <Input type="password" placeholder="******" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="accessSecret"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Secret access key</FormLabel>
                <FormDescription>
                  Secret access key for remote auth
                </FormDescription>
                <FormControl>
                  <Input type="password" placeholder="******" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Separator className="col-span-2" />
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Server username</FormLabel>
                <FormDescription>
                  Username to configure server for auth
                </FormDescription>
                <FormControl>
                  <Input type="text" placeholder="ghosty" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Server password</FormLabel>
                <FormDescription>
                  Password to configure server for auth
                </FormDescription>
                <FormControl>
                  <Input type="password" placeholder="******" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

        </div>
        <div className="flex gap-2 mt-4">
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
      </form>
    </Form >
  )
}
