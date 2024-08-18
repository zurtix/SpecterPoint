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
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { Input } from "@/components/ui/input"
import { ManualServerSchema, manualServerSchema } from "@/components/forms/schemas"

export default function ManualServerForm() {
  const form = useForm<ManualServerSchema>({
    resolver: zodResolver(manualServerSchema),
  })

  function onSubmit(values: ManualServerSchema) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    //
    // use this to invoke to rust backend
    console.log(values)
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div className="grid grid-cols-2 justify-between gap-4">
          <FormField
            control={form.control}
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Name</FormLabel>
                <FormDescription>
                  Identifiable server name
                </FormDescription>
                <FormControl>
                  <Input placeholder="Unique name" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <div className="grid grid-cols-4 gap-2">
            <FormField
              control={form.control}
              name="host"
              render={({ field }) => (
                <FormItem className="col-span-3">
                  <FormLabel>Host</FormLabel>
                  <FormDescription>
                    Host / IP of the existing server
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
                <FormItem>
                  <FormLabel>Port</FormLabel>
                  <FormDescription>
                    Port number
                  </FormDescription>
                  <FormControl>
                    <Input type="number" placeholder="80" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormDescription>
                  Username to authenticate against the server
                </FormDescription>
                <FormControl>
                  <Input placeholder="ghosty" {...field} />
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
                <FormLabel>Username</FormLabel>
                <FormDescription>
                  Auth user to connect to server
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
      </form>
    </Form >
  )
}
