resource "kubernetes_storage_class" "mirror" {
  depends_on = [null_resource.cleanup_leftovers]
  metadata {
    name = "io-engine-nvmf-2"
  }
  storage_provisioner = "io.openebs.csi-mayastor"
  reclaim_policy      = "Delete"
  parameters = {
    repl      = "2"
    protocol  = "nvmf"
    ioTimeout = "30"
  }
}

resource "null_resource" "cleanup_leftovers" {
  provisioner "local-exec" {
    command    = "kubectl delete sc io-engine-nvmf-2"
    on_failure = continue
  }
}
