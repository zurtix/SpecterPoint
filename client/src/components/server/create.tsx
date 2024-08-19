import aws from "@/assets/aws.png"
import azure from "@/assets/azure.png"
import digitalocean from "@/assets/digitalocean.png"
import google from "@/assets/google.png"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs"
import { ResponsiveDialog } from "@/components/ui/responsive-dialog"
import { ManualServer } from "@/components/server/form-manual"
import { AwsServer } from "@/components/server/form-aws"

export function CreateServer({ open, setOpen }: { open: boolean, setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  return (
    <ResponsiveDialog
      isOpen={open}
      setIsOpen={setOpen}
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
          <ManualServer setOpen={setOpen} />
        </TabsContent>
        <TabsContent value="aws">
          <AwsServer />
        </TabsContent>
        <TabsContent value="azure">
        </TabsContent>
        <TabsContent value="digitalocean">
        </TabsContent>
        <TabsContent value="gcp">
        </TabsContent>
      </Tabs>
    </ResponsiveDialog>
  )
}
