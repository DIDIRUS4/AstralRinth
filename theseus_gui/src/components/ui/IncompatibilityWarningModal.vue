<template>
  <Modal
    ref="incompatibleModal"
    :header="t('IncompatibilityWarningModal.Incompatibility')"
    :noblur="!themeStore.advancedRendering"
  >
    <div class="modal-body">
      <p>
        {{ t('IncompatibilityWarningModal.This') }}
        {{ versions?.length > 0 ? t('IncompatibilityWarningModal.Project') : t('IncompatibilityWarningModal.Version') }}
        {{ t('IncompatibilityWarningModal.Warn') }}
      </p>
      <table>
        <tr class="header">
          <th>{{ instance?.metadata.name }}</th>
          <th>{{ projectTitle }}</th>
        </tr>
        <tr class="content">
          <td class="data">
            {{ instance?.metadata.loader }} {{ instance?.metadata.game_version }}
          </td>
          <td>
            <DropdownSelect
              v-if="versions?.length > 1"
              v-model="selectedVersion"
              :options="versions"
              :placeholder="t('IncompatibilityWarningModal.SelectVersion')"
              :name="t('IncompatibilityWarningModal.VersionSelect')"
              :display-name="
                (version) =>
                  `${version?.name} (${version?.loaders
                    .map((name) => formatCategory(name))
                    .join(', ')} - ${version?.game_versions.join(', ')})`
              "
              render-up
            />
            <span v-else>
              <span>
                {{ selectedVersion?.name }} ({{
                  selectedVersion?.loaders.map((name) => formatCategory(name)).join(', ')
                }}
                - {{ selectedVersion?.game_versions.join(', ') }})
              </span>
            </span>
          </td>
        </tr>
      </table>
      <div class="button-group">
        <Button @click="() => incompatibleModal.hide()">
          <XIcon />
          {{ t('IncompatibilityWarningModal.Cancel') }}
        </Button>
        <Button color="primary" :disabled="installing" @click="install()">
          <DownloadIcon />
          {{ installing ? t('IncompatibilityWarningModal.Installing') : t('IncompatibilityWarningModal.Install') }}
        </Button>
      </div>
    </div>
  </Modal>
</template>

<script setup>
import { Button, DownloadIcon, DropdownSelect, formatCategory, Modal, XIcon } from 'omorphia'
import { add_project_from_version as installMod } from '@/helpers/profile'
import { ref } from 'vue'
import { handleError, useTheming } from '@/store/state.js'
import { mixpanel_track } from '@/helpers/mixpanel'
import { i18n } from '@/main.js'

const t = i18n.global.t
const themeStore = useTheming()

const instance = ref(null)
const project = ref(null)
const projectType = ref(null)
const projectTitle = ref(null)
const versions = ref(null)
const selectedVersion = ref(null)
const incompatibleModal = ref(null)
const installing = ref(false)

let markInstalled = () => {
}

defineExpose({
  show: (
    instanceVal,
    projectTitleVal,
    selectedVersions,
    extMarkInstalled,
    projectIdVal,
    projectTypeVal,
  ) => {
    instance.value = instanceVal
    projectTitle.value = projectTitleVal
    versions.value = selectedVersions
    selectedVersion.value = selectedVersions[0]

    project.value = projectIdVal
    projectType.value = projectTypeVal

    incompatibleModal.value.show()
    markInstalled = extMarkInstalled

    mixpanel_track('ProjectInstallStart', { source: 'ProjectIncompatibilityWarningModal' })
  }
})

const install = async () => {
  installing.value = true
  await installMod(instance.value.path, selectedVersion.value.id).catch(handleError)
  installing.value = false
  markInstalled()
  incompatibleModal.value.hide()

  mixpanel_track('ProjectInstall', {
    loader: instance.value.metadata.loader,
    game_version: instance.value.metadata.game_version,
    id: project.value,
    version_id: selectedVersion.value.id,
    project_type: projectType.value,
    title: projectTitle.value,
    source: 'ProjectIncompatibilityWarningModal'
  })
}
</script>

<style lang="scss" scoped>
.data {
  text-transform: capitalize;
}

table {
  width: 100%;
  border-radius: var(--radius-lg);
  border-collapse: collapse;
  box-shadow: 0 0 0 1px var(--color-button-bg);
}

th {
  text-align: left;
  padding: 1rem;
  background-color: var(--color-bg);
  overflow: hidden;
  border-bottom: 1px solid var(--color-button-bg);
}

th:first-child {
  border-top-left-radius: var(--radius-lg);
  border-right: 1px solid var(--color-button-bg);
}

th:last-child {
  border-top-right-radius: var(--radius-lg);
}

td {
  padding: 1rem;
}

td:first-child {
  border-right: 1px solid var(--color-button-bg);
}

.button-group {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;

  :deep(.animated-dropdown .options) {
    max-height: 13.375rem;
  }
}
</style>
