import { useRoute } from 'vue-router';
import { Ref, onMounted, ref, inject, computed, reactive, onActivated } from 'vue';

interface ApiViewResponse {
    viewmodelId: string,
    widgetsData: any
}

export function initializeWidget(props: any) {
    const widgetsData: any = inject("widgetsData");
    const viewmodelId: any = inject("viewmodelId");
    const _data = reactive(Object.assign({}, props));
    const widgetId: string = props.widgetId;
    const data = computed(() => {
        return Object.assign(_data, widgetsData[widgetId]);
    });

    const applyWidgetsData = (newWdgetsData: any) => {
        Object.assign(widgetsData, newWdgetsData);
    }

    const postAction = async (type: string, data: any = undefined) => {
        const fetchResponse = await fetch("/api/action", {
            method: "POST",
            body: JSON.stringify({
                "viewmodelId": viewmodelId.value,
                "widgetId": widgetId,
                "type": type,
                "data": data
            })
        });
        if (fetchResponse.status != 200 || fetchResponse.body == null)
            return Promise.reject("server unexpected response");
        const response = (await fetchResponse.json()) as ApiViewResponse;
        // const response = { viewmodelId: "123", widgetsData: { "yyyyy": { "text": "world" } } };
        applyWidgetsData(response.widgetsData);
    };

    return { props, viewmodelId, data, postAction };
}

export function initializeView() {
    const route = useRoute();
    const viewName = route.params.viewName;
    const viewmodelId = ref("");
    const widgetsData = reactive({});

    const applyWidgetsData = (newWdgetsData: any) => {
        Object.assign(widgetsData, newWdgetsData);
    }

    const mounted = async () => {
        const fetchResponse = await fetch("/api/view/" + viewName, {
            method: "POST",
            body: JSON.stringify({ "viewmodelId": viewmodelId })
        });
        if (fetchResponse.status != 200 || fetchResponse.body == null)
            return Promise.reject("server unexpected response");
        const response = (await fetchResponse.json()) as ApiViewResponse;
        // const response = { viewmodelId: "123", widgetsData: { "xxxxx": { "text": "hello" } } };
        viewmodelId.value = response.viewmodelId;
        applyWidgetsData(response.widgetsData);
    };
    onMounted(mounted);

    return { viewmodelId, widgetsData };
}

export default {
    install: (app: any) => {
        app.config.globalProperties.initializeView = initializeView;
        app.config.globalProperties.initializeWidget = initializeWidget;
    }
};