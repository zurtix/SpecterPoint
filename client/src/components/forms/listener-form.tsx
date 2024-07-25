"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"

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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { MultiBadge } from "@/components/ui/multi-badge"

enum ListenerTypes {
  Http = "Http",
  Https = "Https",
  Tcp = "Tcp",
}

const listenerSchema = z.object({
  type: z.string(),
  name: z.string(),
  host: z.string(),
  port: z.coerce.number(),
  endpoints: z.array(z.string()).optional()
})

export default function ListenerForm() {
  const form = useForm<z.infer<typeof listenerSchema>>({
    resolver: zodResolver(listenerSchema),
  })

  function onSubmit(values: z.infer<typeof listenerSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    console.log(values)
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div className="grid grid-cols-2 justify-between gap-4">
          <div>
            <FormField
              control={form.control}
              name="type"
              render={({ field }) => (
                <FormItem className="space-y-1 mb-2">
                  <FormLabel>Type</FormLabel>
                  <FormDescription>
                    Set the type for the listener
                  </FormDescription>
                  <Select onValueChange={field.onChange} defaultValue={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a listener type" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {
                        (Object.values(ListenerTypes) as Array<keyof typeof ListenerTypes>).map((key) => {
                          return (
                            <SelectItem value={key}>{key}</SelectItem>
                          )
                        })
                      }
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem className="space-y-1 mb-2">
                  <FormLabel>Name</FormLabel>
                  <FormDescription>
                    Unique listener name
                  </FormDescription>
                  <FormControl>
                    <Input placeholder="Unique name" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="host"
              render={({ field }) => (
                <FormItem className="space-y-1 mb-2">
                  <FormLabel>Host</FormLabel>
                  <FormDescription>
                    Host / Server name of the listener
                  </FormDescription>
                  <FormControl>
                    <Input placeholder="0.0.0.0" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="port"
              render={({ field }) => (
                <FormItem className="space-y-1 mb-2">
                  <FormLabel>Port</FormLabel>
                  <FormDescription>
                    Port of where the listener will listen.
                  </FormDescription>
                  <FormControl>
                    <Input placeholder="80" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <div className="w-full">
            <FormField
              control={form.control}
              name="endpoints"
              render={({ field }) => (
                <FormItem className="space-y-1 mb-2">
                  <FormLabel>Endpoints</FormLabel>
                  <FormDescription>
                    Endpoints for the agent to communicate with
                  </FormDescription>
                  <FormControl>
                    <MultiBadge
                      onValueChange={field.onChange}
                      placeholder="Enter endpoints"
                    />
                  </FormControl>
                </FormItem>
              )}
            />
          </div>
        </div>
        <Button type="submit" className="mt-4">Submit</Button>
      </form>
    </Form >
  )
}
