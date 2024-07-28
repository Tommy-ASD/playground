import pathlib
import os
import save
from save import *

import torch
from torch import nn
from torch.nn import Conv2d

from sgm.modules.attention import *

def save_res_block(res_block, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)
    # We can't directly save activation functions, but as they are just attribute of the block,
    # we don't need to save them separately, they will be recreated along with the block.
    
    # saving group normalization layer
    save_group_norm(res_block.in_layers[0], os.path.join(path, 'norm_in'))
    
    # saving the convolutional layer
    save_conv2d(res_block.in_layers[2], os.path.join(path, 'conv_in'))

    # saving the linear layer
    save_linear(res_block.emb_layers[1], os.path.join(path, 'lin_embed'))
    
    # saving group normalization in out_layers
    save_group_norm(res_block.out_layers[0], os.path.join(path, 'norm_out'))

    # saving the convolutional layer in out_layers
    save_conv2d(res_block.out_layers[3], os.path.join(path, 'conv_out'))

    # save skip_connection based on the object type
    if isinstance(res_block.skip_connection, Conv2d):
        save_conv2d(res_block.skip_connection, os.path.join(path, 'skip_connection'))
    elif not isinstance(res_block.skip_connection, nn.Identity):
        assert(False)

def save_cross_attention(cross_attention, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save Linear layers
    save_linear(cross_attention.to_q, os.path.join(path, 'query'))
    save_linear(cross_attention.to_k, os.path.join(path, 'key'))
    save_linear(cross_attention.to_v, os.path.join(path, 'value'))
    save_linear(cross_attention.to_out[0], os.path.join(path, 'out'))

    # Save parameters
    save_scalar(cross_attention.heads, 'n_head', path)

def save_geglu(geglu, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save Linear layers
    save_linear(geglu.proj, os.path.join(path, 'proj'))

def save_feed_forward(feed_forward, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save GEGLU module
    assert(isinstance(feed_forward.net[0], GEGLU))
    save_geglu(feed_forward.net[0], os.path.join(path, 'geglu'))

    # Save Linear layer
    save_linear(feed_forward.net[2], os.path.join(path, 'lin'))


def save_basic_transformer_block(basic_transformer_block, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save CrossAttention, FeedForward and LayerNorm instances
    save_cross_attention(basic_transformer_block.attn1, os.path.join(path, 'attn1'))
    save_feed_forward(basic_transformer_block.ff, os.path.join(path, 'mlp'))
    save_cross_attention(basic_transformer_block.attn2, os.path.join(path, 'attn2'))

    save_layer_norm(basic_transformer_block.norm1, os.path.join(path, 'norm1'))
    save_layer_norm(basic_transformer_block.norm2, os.path.join(path, 'norm2'))  
    save_layer_norm(basic_transformer_block.norm3, os.path.join(path, 'norm3'))  



def save_spatial_transformer(spatial_transformer, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save GroupNorm, Conv2d, BasicTransformerBlock instances
    save_group_norm(spatial_transformer.norm, os.path.join(path, 'norm'))
    #save_conv2d(spatial_transformer.proj_in, os.path.join(path, 'proj_in'))
    save_linear(spatial_transformer.proj_in, os.path.join(path, 'proj_in'))

    save_scalar(len(spatial_transformer.transformer_blocks), 'n_blocks', path)
    for i, block in enumerate(spatial_transformer.transformer_blocks):
        save_basic_transformer_block(block, os.path.join(path, f'transformer_{i}'))
    #save_conv2d(spatial_transformer.proj_out, os.path.join(path, 'proj_out')) 
    save_linear(spatial_transformer.proj_out, os.path.join(path, 'proj_out'))


def save_downsample(downsample, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save Conv2d instance
    save_conv2d(downsample.op, path)

def save_upsample(upsample, path):
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    # Save Conv2d instance
    save_conv2d(upsample.conv, os.path.join(path, 'conv')) 


def save_res_transformer_res(block, path):
    save_res_block(block[0], pathlib.Path(path, 'res1'))
    save_spatial_transformer(block[1], pathlib.Path(path, 'transformer'))
    save_res_block(block[2], pathlib.Path(path, 'res2'))

def save_res_upsample(block, path):
    save_res_block(block[0], pathlib.Path(path, 'res'))
    save_upsample(block[1], pathlib.Path(path, 'upsample'))

def save_res_transformer(block, path):
    save_res_block(block[0], pathlib.Path(path, 'res'))
    save_spatial_transformer(block[1], pathlib.Path(path, 'transformer'))

def save_res_transformer_upsample(block, path):
    save_res_block(block[0], pathlib.Path(path, 'res'))
    save_spatial_transformer(block[1], pathlib.Path(path, 'transformer'))
    save_upsample(block[2], pathlib.Path(path, 'upsample'))

def save_block_type(path, type):
    type_file_path = path / 'type.txt'
    with open(type_file_path, 'w') as f:    # Write block type to text file
        f.write(type)

def save_unet_input_blocks(input_blocks, path):
    save_scalar(9, 'n_blocks', path)

    save_conv2d(input_blocks[0][0], pathlib.Path(path, '0'))
    save_block_type(pathlib.Path(path, '0'), 'conv')

    save_res_block(input_blocks[1][0], pathlib.Path(path, '1'))
    save_block_type(pathlib.Path(path, '1'), 'resnet')

    save_res_block(input_blocks[2][0], pathlib.Path(path, '2'))
    save_block_type(pathlib.Path(path, '2'), 'resnet')

    save_downsample(input_blocks[3][0], pathlib.Path(path, '3'))
    save_block_type(pathlib.Path(path, '3'), 'downsample')

    save_res_transformer(input_blocks[4], pathlib.Path(path, '4'))
    save_block_type(pathlib.Path(path, '4'), 'resnet_transformer')

    save_res_transformer(input_blocks[5], pathlib.Path(path, '5'))
    save_block_type(pathlib.Path(path, '5'), 'resnet_transformer')

    save_downsample(input_blocks[6][0], pathlib.Path(path, '6'))
    save_block_type(pathlib.Path(path, '6'), 'downsample')

    save_res_transformer(input_blocks[7], pathlib.Path(path, '7'))
    save_block_type(pathlib.Path(path, '7'), 'resnet_transformer')

    save_res_transformer(input_blocks[8], pathlib.Path(path, '8'))
    save_block_type(pathlib.Path(path, '8'), 'resnet_transformer')


def save_unet_input_blocks_refiner(input_blocks, path):
    save_scalar(12, 'n_blocks', path)

    save_conv2d(input_blocks[0][0], pathlib.Path(path, '0'))
    save_block_type(pathlib.Path(path, '0'), 'conv')

    save_res_block(input_blocks[1][0], pathlib.Path(path, '1'))
    save_block_type(pathlib.Path(path, '1'), 'resnet')

    save_res_block(input_blocks[2][0], pathlib.Path(path, '2'))
    save_block_type(pathlib.Path(path, '2'), 'resnet')

    save_downsample(input_blocks[3][0], pathlib.Path(path, '3'))
    save_block_type(pathlib.Path(path, '3'), 'downsample')

    save_res_transformer(input_blocks[4], pathlib.Path(path, '4'))
    save_block_type(pathlib.Path(path, '4'), 'resnet_transformer')

    save_res_transformer(input_blocks[5], pathlib.Path(path, '5'))
    save_block_type(pathlib.Path(path, '5'), 'resnet_transformer')

    save_downsample(input_blocks[6][0], pathlib.Path(path, '6'))
    save_block_type(pathlib.Path(path, '6'), 'downsample')

    save_res_transformer(input_blocks[7], pathlib.Path(path, '7'))
    save_block_type(pathlib.Path(path, '7'), 'resnet_transformer')

    save_res_transformer(input_blocks[8], pathlib.Path(path, '8'))
    save_block_type(pathlib.Path(path, '8'), 'resnet_transformer')

    save_downsample(input_blocks[9][0], pathlib.Path(path, '9'))
    save_block_type(pathlib.Path(path, '9'), 'downsample')

    save_res_block(input_blocks[10][0], pathlib.Path(path, '10'))
    save_block_type(pathlib.Path(path, '10'), 'resnet')

    save_res_block(input_blocks[11][0], pathlib.Path(path, '11'))
    save_block_type(pathlib.Path(path, '11'), 'resnet')


def save_unet_output_blocks(output_blocks, path):
    save_scalar(9, 'n_blocks', path)

    save_res_transformer(output_blocks[0], pathlib.Path(path, '0'))
    save_block_type(pathlib.Path(path, '0'), 'resnet_transformer')

    save_res_transformer(output_blocks[1], pathlib.Path(path, '1'))
    save_block_type(pathlib.Path(path, '1'), 'resnet_transformer')

    save_res_transformer_upsample(output_blocks[2], pathlib.Path(path, '2'))
    save_block_type(pathlib.Path(path, '2'), 'resnet_transformer_upsample')

    save_res_transformer(output_blocks[3], pathlib.Path(path, '3'))
    save_block_type(pathlib.Path(path, '3'), 'resnet_transformer')

    save_res_transformer(output_blocks[4], pathlib.Path(path, '4'))
    save_block_type(pathlib.Path(path, '4'), 'resnet_transformer')

    save_res_transformer_upsample(output_blocks[5], pathlib.Path(path, '5'))
    save_block_type(pathlib.Path(path, '5'), 'resnet_transformer_upsample')

    save_res_block(output_blocks[6][0], pathlib.Path(path, '6'))
    save_block_type(pathlib.Path(path, '6'), 'resnet')

    save_res_block(output_blocks[7][0], pathlib.Path(path, '7'))
    save_block_type(pathlib.Path(path, '7'), 'resnet')

    save_res_block(output_blocks[8][0], pathlib.Path(path, '8'))
    save_block_type(pathlib.Path(path, '8'), 'resnet')

def save_unet_output_blocks_refiner(output_blocks, path):
    save_scalar(12, 'n_blocks', path)

    save_res_block(output_blocks[0][0], pathlib.Path(path, '0'))
    save_block_type(pathlib.Path(path, '0'), 'resnet')

    save_res_block(output_blocks[1][0], pathlib.Path(path, '1'))
    save_block_type(pathlib.Path(path, '1'), 'resnet')

    save_res_upsample(output_blocks[2], pathlib.Path(path, '2'))
    save_block_type(pathlib.Path(path, '2'), 'resnet_upsample')

    save_res_transformer(output_blocks[3], pathlib.Path(path, '3'))
    save_block_type(pathlib.Path(path, '3'), 'resnet_transformer')

    save_res_transformer(output_blocks[4], pathlib.Path(path, '4'))
    save_block_type(pathlib.Path(path, '4'), 'resnet_transformer')

    save_res_transformer_upsample(output_blocks[5], pathlib.Path(path, '5'))
    save_block_type(pathlib.Path(path, '5'), 'resnet_transformer_upsample')

    save_res_transformer(output_blocks[6], pathlib.Path(path, '6'))
    save_block_type(pathlib.Path(path, '6'), 'resnet_transformer')

    save_res_transformer(output_blocks[7], pathlib.Path(path, '7'))
    save_block_type(pathlib.Path(path, '7'), 'resnet_transformer')

    save_res_transformer_upsample(output_blocks[8], pathlib.Path(path, '8'))
    save_block_type(pathlib.Path(path, '8'), 'resnet_transformer_upsample')

    save_res_block(output_blocks[9][0], pathlib.Path(path, '9'))
    save_block_type(pathlib.Path(path, '9'), 'resnet')

    save_res_block(output_blocks[10][0], pathlib.Path(path, '10'))
    save_block_type(pathlib.Path(path, '10'), 'resnet')

    save_res_block(output_blocks[11][0], pathlib.Path(path, '11'))
    save_block_type(pathlib.Path(path, '11'), 'resnet')


'''def save_unet_input_blocks(input_blocks, path):
    save_conv2d(input_blocks[0][0], pathlib.Path(path, 'conv'))
    save_res_transformer(input_blocks[1], pathlib.Path(path, 'rt1'))
    save_res_transformer(input_blocks[2], pathlib.Path(path, 'rt2'))
    save_downsample(input_blocks[3][0], pathlib.Path(path, 'd1'))
    save_res_transformer(input_blocks[4], pathlib.Path(path, 'rt3'))
    save_res_transformer(input_blocks[5], pathlib.Path(path, 'rt4'))
    save_downsample(input_blocks[6][0], pathlib.Path(path, 'd2'))
    save_res_transformer(input_blocks[7], pathlib.Path(path, 'rt5'))
    save_res_transformer(input_blocks[8], pathlib.Path(path, 'rt6'))
    save_downsample(input_blocks[9][0], pathlib.Path(path, 'd3'))
    save_res_block(input_blocks[10][0], pathlib.Path(path, 'r1'))
    save_res_block(input_blocks[11][0], pathlib.Path(path, 'r2'))

def save_unet_output_blocks(output_blocks, path):
    save_res_block(output_blocks[0][0], pathlib.Path(path, 'r1'))
    save_res_block(output_blocks[1][0], pathlib.Path(path, 'r2'))
    save_res_upsample(output_blocks[2], pathlib.Path(path, 'ru'))
    save_res_transformer(output_blocks[3], pathlib.Path(path, 'rt1'))
    save_res_transformer(output_blocks[4], pathlib.Path(path, 'rt2'))
    save_res_transformer_upsample(output_blocks[5], pathlib.Path(path, 'rtu1'))
    save_res_transformer(output_blocks[6], pathlib.Path(path, 'rt3'))
    save_res_transformer(output_blocks[7], pathlib.Path(path, 'rt4'))
    save_res_transformer_upsample(output_blocks[8], pathlib.Path(path, 'rtu2'))
    save_res_transformer(output_blocks[9], pathlib.Path(path, 'rt5'))
    save_res_transformer(output_blocks[10], pathlib.Path(path, 'rt6'))
    save_res_transformer(output_blocks[11], pathlib.Path(path, 'rt7'))'''

def save_unet_model(model, path, refiner=False):
    with torch.no_grad():
        pathlib.Path(path).mkdir(parents=True, exist_ok=True)
        save_scalar(model.model_channels, 'model_channels', path)
        save_linear(model.time_embed[0], pathlib.Path(path, 'lin1_time_embed'))
        save_linear(model.time_embed[2], pathlib.Path(path, 'lin2_time_embed'))
        save_linear(model.label_emb[0][0], pathlib.Path(path, 'lin1_label_embed'))
        save_linear(model.label_emb[0][2], pathlib.Path(path, 'lin2_label_embed'))
        if refiner:
            save_unet_input_blocks_refiner(model.input_blocks, pathlib.Path(path, 'input_blocks'))
        else: 
            save_unet_input_blocks(model.input_blocks, pathlib.Path(path, 'input_blocks'))
        save_res_transformer_res(model.middle_block, pathlib.Path(path, 'middle_block'))
        if refiner:
            save_unet_output_blocks_refiner(model.output_blocks, pathlib.Path(path, 'output_blocks'))
        else:
            save_unet_output_blocks(model.output_blocks, pathlib.Path(path, 'output_blocks'))
        save_group_norm(model.out[0], pathlib.Path(path, 'norm_out'))
        save_conv2d(model.out[2], pathlib.Path(path, 'conv_out'))