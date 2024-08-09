import { Link } from "@tanstack/react-router"
import ghosty from '@/assets/ghosty.gif'
import aws from "@/assets/aws.png"
import azure from "@/assets/azure.png"
import digitalocean from "@/assets/digitalocean.png"
import google from "@/assets/google.png"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs"
import { ResponsiveDialog } from '@/components/ui/responsive-dialog'
import { useState } from 'react'
import ListenerForm from '@/components/forms/listener-form'
import ManualServerForm from '@/components/forms/manual-server-form'
import AwsServerForm from '@/components/forms/aws-server.form'
import { invoke } from '@tauri-apps/api/tauri'

export function Navigation() {
  const [isCreateListenerOpen, setIsCreateListenerOpen] = useState(false);
  const [isCreateServerOpen, setIsCreateServerOpen] = useState(false);

  return (
    <div>
      <ResponsiveDialog
        isOpen={isCreateListenerOpen}
        setIsOpen={setIsCreateListenerOpen}
        title="Create new Listener"
        description="Fill out the form below to create a new listener"
      >
        <ListenerForm />
      </ResponsiveDialog>
      <ResponsiveDialog
        isOpen={isCreateServerOpen}
        setIsOpen={setIsCreateServerOpen}
        title="Create new Server"
        description="Select an option to deploy your server, fill out required criteria"
      >
        <Tabs defaultValue="manual">
          <TabsList className="grid w-full grid-cols-5">
            <TabsTrigger value="manual">Manual</TabsTrigger>
            <TabsTrigger value="aws" className="gap-2">
              <img src={aws} height={20} width={20} className='bg-white rounded ronded-lg' />
              AWS
            </TabsTrigger>
            <TabsTrigger value="azure" className="gap-2">
              <img src={azure} height={20} width={20} className='bg-white rounded ronded-lg' />
              Azure
            </TabsTrigger>
            <TabsTrigger value="digitalocean" className="gap-2">
              <img src={digitalocean} height={20} width={20} className='bg-white rounded ronded-lg' />
              Digital Ocean
            </TabsTrigger>
            <TabsTrigger value="gcp" className="gap-2">
              <img src={google} height={20} width={20} className='bg-white rounded ronded-lg' />
              Google
            </TabsTrigger>
          </TabsList>
          <TabsContent value="manual">
            <ManualServerForm />
          </TabsContent>
          <TabsContent value="aws">
            <AwsServerForm />
          </TabsContent>
          <TabsContent value="azure">
          </TabsContent>
          <TabsContent value="digitalocean">
          </TabsContent>
          <TabsContent value="gcp">
          </TabsContent>
        </Tabs>
      </ResponsiveDialog>

      <div className="z-70 p-2 fixed bottom-0 left-0 w-full">
        <hr className='mb-1' />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button className='bg-secondary'>
              <img src={ghosty} alt="ghost" width={40} height={40} />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-56" align='start' alignOffset={2}>
            <DropdownMenuLabel>Menu</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                <span>Listeners</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem asChild>
                    <span onClick={() => setIsCreateListenerOpen(true)}>Create</span>
                  </DropdownMenuItem>
                  <DropdownMenuItem asChild>
                    <Link to="/listeners/view" className='w-full'>View</Link>
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                <span>Servers</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem asChild>
                    <span onClick={() => setIsCreateServerOpen(true)}>Create</span>
                  </DropdownMenuItem>
                  <DropdownMenuItem asChild>
                    <Link to="/servers/view" className='w-full'>View</Link>
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
            <DropdownMenuItem asChild>
              <Link to="/targets" className='w-full'>Targets</Link>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem asChild>
              <Link to='/blacklist'>Blacklist</Link>
            </DropdownMenuItem>
            <DropdownMenuItem asChild>
              <Link to='/configuration'>Configuration</Link>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem asChild>
              <a target="_blank" href="https://github.com/Surphix/SpecterPoint" className='w-full'>GitHub</a>
            </DropdownMenuItem>
            <DropdownMenuItem disabled>
              <span>Support</span>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
              <span className="w-full" onClick={() => invoke("quit")}>Quit</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  )
}
