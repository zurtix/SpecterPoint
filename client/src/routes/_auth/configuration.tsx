import ConfigureProxyForm from '@/components/configuration/configuration-proxy'
import { createFileRoute } from '@tanstack/react-router'
import { Switch } from "@/components/ui/switch"
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"
import { Label } from "@/components/ui/label"

export const Route = createFileRoute('/_auth/configuration')({
  component: () => <Configuration />
})

const configuration = {
  "proxy": {
    "server": "something",
    "username": "something",
    "enabled": true
  }
}

function Configuration() {
  function handleSwitch(enabled: boolean, config: string) {
    console.log("Setting: " + config + " to " + enabled);
  }

  return (
    <div className="p-6">
      <Accordion type="single" collapsible className="w-full">
        <AccordionItem value="proxy">
          <div className="flex gap-4">
            <Switch
              defaultChecked={configuration.proxy.enabled}
              onCheckedChange={(enabled) => handleSwitch(enabled, "proxy")}
              className="mt-3"
            />
            <AccordionTrigger>
              <Label>Proxy</Label>
            </AccordionTrigger>
          </div>
          <AccordionContent>
            <ConfigureProxyForm />
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  )
}
