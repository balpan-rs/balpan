use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_declaration_of_function() {
    let source_code = indoc! { r#"
    list *listCreate(void);
    void listRelease(list *list);
    void listEmpty(list *list);
    list *listAddNodeHead(list *list, void *value);
    list *listAddNodeTail(list *list, void *value);
    list *listInsertNode(list *list, listNode *old_node, void *value, int after);
    void listDelNode(list *list, listNode *node);
    listIter *listGetIterator(list *list, int direction);
    listNode *listNext(listIter *iter);
    void listReleaseIterator(listIter *iter);
    list *listDup(list *orig);
    listNode *listSearchKey(list *list, void *key);
    listNode *listIndex(list *list, long index);
    void listRewind(list *list, listIter *li);
    void listRewindTail(list *list, listIter *li);
    void listRotateTailToHead(list *list);
    void listRotateHeadToTail(list *list);
    void listJoin(list *l, list *o);
    void listInitNode(listNode *node, void *value);
    void listLinkNodeHead(list *list, listNode *node);
    void listLinkNodeTail(list *list, listNode *node);
    void listUnlinkNode(list *list, listNode *node);"#};

    let result = indoc! { r#"
    list *listCreate(void);
    void listRelease(list *list);
    void listEmpty(list *list);
    list *listAddNodeHead(list *list, void *value);
    list *listAddNodeTail(list *list, void *value);
    list *listInsertNode(list *list, listNode *old_node, void *value, int after);
    void listDelNode(list *list, listNode *node);
    listIter *listGetIterator(list *list, int direction);
    listNode *listNext(listIter *iter);
    void listReleaseIterator(listIter *iter);
    list *listDup(list *orig);
    listNode *listSearchKey(list *list, void *key);
    listNode *listIndex(list *list, long index);
    void listRewind(list *list, listIter *li);
    void listRewindTail(list *list, listIter *li);
    void listRotateTailToHead(list *list);
    void listRotateHeadToTail(list *list);
    void listJoin(list *l, list *o);
    void listInitNode(listNode *node, void *value);
    void listLinkNodeHead(list *list, listNode *node);
    void listLinkNodeTail(list *list, listNode *node);
    void listUnlinkNode(list *list, listNode *node);"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}

#[ignore]
fn test_function_definition_together_with_macro_combined() {
    let source_code = indoc! {r#"
    REDIS_NO_SANITIZE("bounds")
    clusterMsgSendBlock *clusterCreatePublishMsgBlock(robj *channel, robj *message, uint16_t type) {

        uint32_t channel_len, message_len;

        channel = getDecodedObject(channel);
        message = getDecodedObject(message);
        channel_len = sdslen(channel->ptr);
        message_len = sdslen(message->ptr);

        size_t msglen = sizeof(clusterMsg)-sizeof(union clusterMsgData);
        msglen += sizeof(clusterMsgDataPublish) - 8 + channel_len + message_len;
        clusterMsgSendBlock *msgblock = createClusterMsgSendBlock(type, msglen);

        clusterMsg *hdr = &msgblock->msg;
        hdr->data.publish.msg.channel_len = htonl(channel_len);
        hdr->data.publish.msg.message_len = htonl(message_len);
        memcpy(hdr->data.publish.msg.bulk_data,channel->ptr,sdslen(channel->ptr));
        memcpy(hdr->data.publish.msg.bulk_data+sdslen(channel->ptr),
            message->ptr,sdslen(message->ptr));

        decrRefCount(channel);
        decrRefCount(message);
        
        return msgblock;
    }"#};

    let result = indoc! {r#"
    /// [TODO] clusterCreatePublishMsgBlock
    REDIS_NO_SANITIZE("bounds")
    clusterMsgSendBlock *clusterCreatePublishMsgBlock(robj *channel, robj *message, uint16_t type) {

        uint32_t channel_len, message_len;

        channel = getDecodedObject(channel);
        message = getDecodedObject(message);
        channel_len = sdslen(channel->ptr);
        message_len = sdslen(message->ptr);

        size_t msglen = sizeof(clusterMsg)-sizeof(union clusterMsgData);
        msglen += sizeof(clusterMsgDataPublish) - 8 + channel_len + message_len;
        clusterMsgSendBlock *msgblock = createClusterMsgSendBlock(type, msglen);

        clusterMsg *hdr = &msgblock->msg;
        hdr->data.publish.msg.channel_len = htonl(channel_len);
        hdr->data.publish.msg.message_len = htonl(message_len);
        memcpy(hdr->data.publish.msg.bulk_data,channel->ptr,sdslen(channel->ptr));
        memcpy(hdr->data.publish.msg.bulk_data+sdslen(channel->ptr),
            message->ptr,sdslen(message->ptr));

        decrRefCount(channel);
        decrRefCount(message);
        
        return msgblock;
    }"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}
